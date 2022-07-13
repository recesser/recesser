VAULT_ADDR="http://vault.vault:8200"
KUBERNETES_HOST="https://kubernetes.default.svc.cluster.local"

ROOT_TTL="87600h"
INT_TTL="43800h"

##
# ClusterIssuer
##

DOMAIN="recesser.com"
VAULT_ROLE="recesser"

# Generate root certificate

vault secrets disable pki
vault secrets enable pki
vault secrets tune -max-lease-ttl="${ROOT_TTL}" pki

vault write -field=certificate pki/root/generate/internal \
    common_name="${DOMAIN}" \
    ttl="${ROOT_TTL}" > root.crt

vault write pki/config/urls \
    issuing_certificates="${VAULT_ADDR}/v1/pki/ca" \
    crl_distribution_points="${VAULT_ADDR}/v1/pki/crl"

# Generate intermediate certificate

vault secrets disable pki_int
vault secrets enable -path=pki_int pki
vault secrets tune -max-lease-ttl="${INT_TTL}" pki_int

vault write -field=csr pki_int/intermediate/generate/internal \
    common_name="${DOMAIN} Intermediate Authority" \
    ttl="${INT_TTL}" > intermediate.csr

# Sign intermediate certificate with root certificate

vault write -field=certificate pki/root/sign-intermediate \
    csr=@intermediate.csr \
    ttl="${INT_TTL}" > intermediate.crt

vault write pki_int/intermediate/set-signed \
    certificate=@intermediate.crt

# Create role

vault write pki_int/roles/"${VAULT_ROLE}" \
    allowed_domains="${DOMAIN}" \
    allow_subdomains=true \
    max_ttl=72h

# Create policy

vault policy write pki_int - <<EOF
path "pki_int*"                      { capabilities = ["read", "list"] }
path "pki_int/roles/${VAULT_ROLE}"   { capabilities = ["create", "update"] }
path "pki_int/sign/${VAULT_ROLE}"    { capabilities = ["create", "update"] }
path "pki_int/issue/${VAULT_ROLE}"   { capabilities = ["create"] }
EOF

trust anchor --store root.crt

# Retrieve kubernetes root certificate

kubectl exec -n vault vault-0 -- \
    cat /var/run/secrets/kubernetes.io/serviceaccount/ca.crt > kubernetes.crt

# Retrieve vault service account token

kubectl exec -n vault vault-0 -- \
    cat /var/run/secrets/kubernetes.io/serviceaccount/token > token

# Enable kubernetes auth

vault auth disable kubernetes
vault auth enable kubernetes

vault write auth/kubernetes/config \
    kubernetes_host="${KUBERNETES_HOST}" \
    kubernetes_ca_cert=@kubernetes.crt \
    token_reviewer_jwt=@token

vault write auth/kubernetes/role/clusterissuer \
    bound_service_account_names=vault-clusterissuer \
    bound_service_account_namespaces=cert-manager \
    policies=pki_int \
    ttl=20m

# Enable jwt auth using kubernetes as OIDC provider (does not currently work with cert-manager)
# See https://github.com/jetstack/cert-manager/issues/4144

vault auth disable jwt
vault auth enable jwt

vault write auth/jwt/config \
    oidc_discovery_url="${KUBERNETES_HOST}" \
    oidc_discovery_ca_pem=@kubernetes.crt

vault write auth/jwt/role/clusterissuer \
    role_type="jwt" \
    bound_audiences="${KUBERNETES_HOST}" \
    user_claim="sub" \
    bound_subject="system:serviceaccount:cert-manager:vault-clusterissuer" \
    policies=pki_int \
    ttl=20m
