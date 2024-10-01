use std::process::{Command, exit};
use names::Generator;

fn commit_push(branch: &str) {
    // Étape 1: git add
    let add = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add command"); 

    if !add.status.success() {
        let error_message = String::from_utf8_lossy(&add.stderr);
        eprintln!("Error: Failed to add files to the git repo. Details: {}", error_message);
        exit(1); 
    }

    // Étape 2: git commit avec message généré automatiquement
    let commit = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(generator_name())
        .output()
        .expect("Error: Failed to execute git commit"); 

    if !commit.status.success() {
        let error_message = String::from_utf8_lossy(&commit.stderr);
        eprintln!("Error: Failed to commit changes. Details: {}", error_message);
        exit(1); 
    }

    // Étape 3: git push sur la branche spécifiée
    let push = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg(branch)
        .output()
        .expect("Error: Failed to execute git push"); 

    if !push.status.success() {
        let error_message = String::from_utf8_lossy(&push.stderr);
        eprintln!("Error: Failed to push changes to remote. Details: {}", error_message);
        exit(1); 
    }

    println!("Successfully added, committed, and pushed all changes to branch {}", branch);
}

fn generator_name() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}

fn main() {
    // Appel de la fonction avec la branche par défaut 'main' ou 'master'
    let branch = "main";  
    commit_push(branch);
}
