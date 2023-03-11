Proyecto personal para profundizar conocimientos en GTK4 con RUST. Mi objetivo es aprovechar al maximo la biblioteca, utilizando todas las funcionalidades que ofrece

# irc-client

El cliente se implemento siguiendo el protocolo RFC 1459. Esta particularmente diseñado para funcionar con la implementación del respositorio _link_, pero se puede facilmente adaptar a otros servidores, cambiando ligeramente el manejo de mensajes.

## Utilizacion

Para iniciar la interfaz grafica, se puede ejecutar el siguiente comando
```bash
cargo run
```

Para correr el programa utilizando la funcionalidad de registración automatica (localhost, puerto 800, nickname aleatorio), se puede incluir el _feature_ `automatic-login`. De la siguiente forma
```bash
cargo run --features automatic-login
```

## Funcionalidad

- [x] Registración en el sevidor
- [x] Envio de mensajes privados
- [x] Union y envio de mensajes a canales
- [x] Eliminación de chats
- [ ] Partida de canales
- [ ] Cambio de nickname
