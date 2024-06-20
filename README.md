# API Rest para la aplicación vida submarina
## Descripción
API Rest para la aplicación vida submarina, la cual permite a los usuarios visualizar información sobre la vida submarina. También permite a los usuarios registrarse y loguearse para poder acceder a la información. Además de acceder a noticias de actualidad relacionadas sobre el tema

## Tecnologías
- Rust
- Actix-web
- MongoDB

## Requisitos
- Rust 1.51.0
- MongoDB 4.4.5
- Cargo 1.51.0

## Instalación
1. Clonar el repositorio
```bash
git clone https://github.com/LuisFerRodVar/api-vida-submarina-app.git
```
2. Ejecutar el proyecto
```bash
cd api-vida-submarina-app
cargo run
```

## Endpoints

/api/users (POST): Permite registrar un usuario
```json
{
    "email": "email",
    "password": "password"
    "email_2": ""
    "username": "username",
}
```
/api/login (POST): Permite loguear un usuario
```json
{

    "email": "email",
    "password": "password"
    "username": "",
    "email_2": ""
}
```
/api/news (POST): Permite agregar noticias
```json
{
    "title": "title",
    "content": "description",
    "thumbnail": "image",
}
```
/api/news (GET): Permite obtener todas las noticias

/api/species (POST): Permite agregar especies
```json
{
    "name": "name",
    "description": "description",
    "thumbnail": "image",
}
```
/api/species (GET): Permite obtener todas las especies

## Licencia

[GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html)
