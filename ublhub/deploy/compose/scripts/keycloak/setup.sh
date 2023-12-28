#!/bin/sh -l
KEYCLOAK_HOME="/opt/keycloak"

while ! $KEYCLOAK_HOME/bin/kcadm.sh config credentials config --server "$KEYCLOAK_SERVER_URL" --realm master --user "$KEYCLOAK_ADMIN" --password "$KEYCLOAK_ADMIN_PASSWORD" &> /dev/null; do
  echo "Waiting for Keycloak to start up..."
  sleep 3
done

$KEYCLOAK_HOME/bin/kcadm.sh create realms -s realm=ublhub -s enabled=true -o
$KEYCLOAK_HOME/bin/kcadm.sh create clients -r ublhub  -f - << EOF
  {
    "clientId": "ublhub-api",
    "secret": "secret"
  }
EOF
