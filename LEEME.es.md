# Combina

Utilidad de línea de comandos escrita en Rust para combinar las líneas de dos archivos (prefijos y sufijos) en todas las combinaciones posibles, con un separador opcional entre ambos.

## Uso

```sh
combina [OPCIONES] <ficheroprefijos> <ficherosufijos>
```

### Opciones

- `-s`, `--separador <cad>`: Separador entre el prefijo y el sufijo.

### Argumentos

- `<ficheroprefijos>`: Ruta al archivo que contiene las líneas de prefijo.
- `<ficherosufijos>`: Ruta al archivo que contiene las líneas de sufijo.

## Ejemplos

### Prefijos y sufijos directamente

Combina las líneas de dos archivos sin un separador:

```sh
./combina archivo1.txt archivo2.txt
```

### Usando un separador

Combina las líneas de dos archivos con un guion como separador:

```sh
./combina -s "-" archivo1.txt archivo2.txt
```

O puedes proporcionar la opción del separador al final:

```sh
./combina archivo1.txt archivo2.txt -s "-"
```

## Instalación

### Construcción desde el Código Fuente

1. Clona el repositorio:

    ```sh
    git clone https://github.com/jordiroca/combina-rust.git
    cd combina-rust
    ```

2. Compila:

    ```sh
    cargo build --release
    ```

3. El binario estará en `target/release`. Puedes ejecutarlo directamente:

    ```sh
    ./target/release/combina
    ```

## Licencia

Ver el archivo [LICENCIA](LICENCIA).
