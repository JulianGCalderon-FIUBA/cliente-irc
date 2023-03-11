Proyecto personal para profundizar conocimientos en GTK4 con RUST. Mi objetivo es aprovechar al máximo la biblioteca, utilizando todas las funcionalidades que ofrece

# irc-client

El cliente se implementó siguiendo el protocolo RFC 1459. Está particularmente diseñado para funcionar con la implementación del servidor [irc-server](https://github.com/JulianGCalderon/irc-server), pero se puede fácilmente adaptar a otros servidores, cambiando ligeramente el manejo de mensajes.

## Utilización

Para iniciar la interfaz grafica, se puede ejecutar el siguiente comando
```bash
cargo run
```

Para correr el programa utilizando la funcionalidad de registración automática (localhost, puerto 9000, nickname aleatorio), se puede incluir el _feature_ `automatic-login`. De la siguiente forma
```bash
cargo run --features automatic-login
```

## Funcionalidad

- [x] Registración en el servidor
- [x] Envio de mensajes privados
- [x] Union y envío de mensajes a canales
- [x] Eliminación de chats
- [ ] Partida de canales
- [ ] Cambio de nickname
