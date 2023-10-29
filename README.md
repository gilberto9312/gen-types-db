# Generar types de typescript

#### Puedes generar types para typescript dada una tabla de postrgresql

Este es un proyecto Rust que utiliza Diesel para interactuar con una base de datos PostgreSQL. 


## Requisitos previos

Antes de comenzar, asegúrate de tener instalado lo siguiente:

- Rust: Sigue las instrucciones en [la página oficial de Rust](https://www.rust-lang.org/tools/install) para instalar Rust y Cargo en tu sistema.
- PostgreSQL: Asegúrate de tener una instancia de PostgreSQL en ejecución. Puedes instalar PostgreSQL siguiendo las instrucciones en [la página oficial de PostgreSQL](https://www.postgresql.org/download/).

## Configuración

1. Clona este repositorio en tu máquina local.

    ```bash
    git clone https://github.com/gilberto9312/gen-types-db
    cd gen-types-db
    ```


## Ejecución

Para ejecutar el proyecto, utiliza el siguiente comando:

```bash
cargo run [CADENA_DE_CONEXION] [NOMBRE_TABLA]
```

