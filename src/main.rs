mod models;
mod state;
mod handlers;

use handlers::create::create_data;
use handlers::delete::delete_data;
use handlers::read::{read_all_data, read_data};
use handlers::update::update_data;
use tide::Request;
use handlers::auth::login;


#[async_std::main]
async fn main() -> tide::Result<()> {
    // Cria o estado global da aplicação
    let state = state::new_state();

    // Cria o app Tide e associa o estado
    let mut app = tide::with_state(state);

    // Define as rotas CRUD
    app.at("/data").post(create_data); // Cria
    app.at("/data").get(read_all_data); // Lê todos
    app.at("/data/:id").get(read_data); // Lê um
    app.at("/data/:id").put(update_data); // Atualiza
    app.at("/data/:id").delete(delete_data); // Deleta
    app.at("/login").post(login); // Login

    let addr = "127.0.0.1:8080";
    println!("Servidor CRUD rodando em: http://{addr}");

    // Inicia o servidor
    app.listen(addr).await?;
    Ok(())
}