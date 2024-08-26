/*mod project;
mod user;
mod platform;

use project::Project;
use user::User;
use platform::Platform;

fn main() {
    let mut platform = Platform::new();
    let project1 = Project::new("Green Energy", "A project for sustainable energy", 1000, 500);
    let project2 = Project::new("Tech Innovation", "A project for new tech", 1500, 750);

    platform.add_project(project1);
    platform.add_project(project2);

    let mut user = User::new("Alice", 700);
    user.buy_tokens(&mut platform.projects[0], 600);

    for project in platform.get_projects() {
        println!("Project listed: {:?}", project);
    }
}
*/
mod project;
mod user;
mod platform;

use project::Project;
use user::User;
use platform::Platform;

fn main() {
    // Créer une plateforme avec des projets
    let mut platform = Platform::new();
    let project1 = Project::new("Green Energy", "A project for sustainable energy", 1000, 500);
    let project2 = Project::new("Tech Innovation", "A project for new tech", 1500, 750);

    platform.add_project(project1);
    platform.add_project(project2);

    // Créer un utilisateur
    let mut user = User::new("Alice", 700);

    // Afficher l'état initial de l'utilisateur
    println!("User before purchase: {:?}", user);

    // Acheter des tokens pour le premier projet
    if user.buy_tokens(&mut platform.projects[0], 600) {
        println!("User successfully bought tokens.");
    } else {
        println!("User failed to buy tokens.");
    }

    // Afficher l'état de l'utilisateur après l'achat
    println!("User after purchase: {:?}", user);

    // Rembourser des tokens
    if user.refund_tokens(&mut platform.projects[0], 200) {
        println!("User successfully refunded tokens.");
    } else {
        println!("User failed to refund tokens.");
    }

    // Afficher l'état de l'utilisateur après le remboursement
    println!("User after refund: {:?}", user);

    // Afficher les projets de la plateforme
    for project in platform.get_projects() {
        println!("Project listed: {:?}", project);
    }
}

