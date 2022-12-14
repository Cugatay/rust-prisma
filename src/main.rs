mod prisma;

use dotenv::dotenv;
use prisma::{user, PrismaClient};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = PrismaClient::_builder().build().await.unwrap();

    let user = cli.user().create("asd".to_owned(), vec![]).exec().await;
    println!("{:?}", user);

    let asd = cli
        .user()
        .find_first(vec![user::display_name::equals("asd".to_owned())])
        .exec()
        .await;

    println!("{:?}", asd);
}
