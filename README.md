# MolGen JSON to Kafka

Diese Anwendung versendet MV64e HTTP requests mit DNPM V2.1 Payload an einen Kafka Broker

## Konfiguration

Beim Start der Anwendung können Parameter angegeben werden.

```
Usage: molgen-to-kafka [OPTIONS] --token <TOKEN>

Options:
      --listen <LISTEN>
          Address and port for HTTP requests [env: LISTEN=] [default: [::]:3000]
      --token <TOKEN>
          bcrypt hashed Security Token [env: SECURITY_TOKEN=]
      --bootstrap-server <BOOTSTRAP_SERVER>
          Kafka Bootstrap Server [env: KAFKA_BOOTSTRAP_SERVERS=] [default: kafka:9094]
      --topic <TOPIC>
          Kafka Topic [env: KAFKA_TOPIC=] [default: etl-processor_input]
      --ssl-ca-file <SSL_CA_FILE>
          CA file for SSL connection to Kafka [env: KAFKA_SSL_CA_FILE=]
      --ssl-cert-file <SSL_CERT_FILE>
          Certificate file for SSL connection to Kafka [env: KAFKA_SSL_CERT_FILE=]
      --ssl-key-file <SSL_KEY_FILE>
          Key file for SSL connection to Kafka [env: KAFKA_SSL_KEY_FILE=]
      --ssl-key-password <SSL_KEY_PASSWORD>
          The SSL key password [env: KAFKA_SSL_KEY_PASSWORD=]
```

Die Anwendung lässt sich auch mit Umgebungsvariablen konfigurieren.

* `LISTEN`: Adresse und Port für eingehende HTTP-Requests. Standardwert: `[::]:3000` - Port `3000` auf allen
  Adressen (IPv4 und IPv6)
* `SECURITY_TOKEN`: Verpflichtende Angabe des Benutzernamens und *bcrypt*-Hash des Passworts
* `KAFKA_BOOTSTRAP_SERVERS`: Zu verwendende Kafka-Bootstrap-Server als kommagetrennte Liste
* `KAFKA_TOPIC`: Zu verwendendes Topic zum Warten auf neue Anfragen. Standardwert: `etl-processor_input`

Optionale Umgebungsvariablen - wenn angegeben wird eine SSL-Verbindung zu Kafka aufgebaut.

* `KAFKA_SSL_CA_FILE`: CA für SSL-Verbindungen
* `KAFKA_SSL_CERT_FILE`: SSL Certificate Datei
* `KAFKA_SSL_KEY_FILE`: SSL Key Datei
* `KAFKA_SSL_KEY_PASSWORD`: SSL KEY Passwort (wenn benötigt)