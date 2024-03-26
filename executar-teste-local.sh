#!/usr/bin/env bash

# Use este script para executar testes locais

RESULTS_WORKSPACE="$GATLING_HOME/results"
GATLING_BIN_DIR=$GATLING_HOME/bin
GATLING_WORKSPACE="$GATLING_HOME/user-files"

runGatling() {
    sh $GATLING_BIN_DIR/gatling.sh -rm local -s RinhaBackendCrebitosSimulation \
        -rd "Rinha de Backend - 2024/Q1: Crébito" \
        -rf $RESULTS_WORKSPACE \
        -sf "$GATLING_WORKSPACE/simulations"
}

startTest() {
    for i in {1..2}; do
        # 2 requests to wake the 2 api instances up :)
        curl --fail http://localhost:9999/clientes/1/extrato && \
        echo "" && \
        curl --fail http://localhost:9999/clientes/1/extrato && \
        echo "" && \
        runGatling && \
        break || sleep 2;
    done
    PGPASSWORD=postgres PGUSER=postgres PGHOST=localhost PGDATABASE=rinha_backend_rust psql -c "UPDATE clients SET balance = 0 WHERE 1=1; DELETE FROM transactions WHERE 1=1;"
}

startTest
