
# workflow-with-rust


Ce projet est un outil en ligne de commande (CLI) écrit en Rust pour automatiser les étapes Git courantes, comme l'ajout de fichiers, le commit avec un message généré automatiquement, et le push vers une branche spécifiée. Cet outil permet de simplifier et d'accélérer le workflow Git en une seule commande.

## Fonctionnalités

- **git add** : Ajoute tous les fichiers modifiés au suivi de Git (`git add -A`).
- **git commit** : Fait un commit avec un message généré automatiquement en utilisant la crate `names`, qui génère des noms uniques.
- **git push** : Pousse les modifications sur une branche distante (`origin` par défaut, branche configurable).

## Prérequis

Avant de commencer, assurez-vous d'avoir installé les éléments suivants :

- **Rust** : Vous devez avoir installé Rust pour compiler et exécuter ce projet. Vous pouvez installer Rust en suivant les instructions sur [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Git** : L'outil Git doit être installé pour que les commandes `git` fonctionnent correctement.

## Installation

1. Clonez le dépôt dans votre environnement local :

   ```bash
   git clone https://github.com/thekrauss/workflow-with-rust
.git
   ```

2. Accédez au dossier du projet :

   ```bash
   cd workflow-with-rust

   ```

3. Ajoutez la dépendance `names` dans votre fichier `Cargo.toml` si elle n'est pas déjà présente :

   ```toml
   [dependencies]
   names = "0.13.0"
   ```

4. Compilez le projet en mode release :

   ```bash
    $ cargo build --release
   ```
   puis

   ``` 
    $ cargo  install--path . 
   ```                                                                                                                            


## Utilisation

1. Une fois le projet compilé, exécutez l'outil en utilisant la commande suivante :

   ```bash
   workflow
   ```

2. Le programme exécutera les actions suivantes :

   - Ajouter tous les fichiers non suivis ou modifiés (`git add -A`).
   - Effectuer un commit avec un message généré automatiquement.
   - Pousser les changements vers la branche `main` (ou une autre branche si spécifiée dans le code).

### Exécution sur une branche spécifique

Le programme est configuré pour pousser les changements sur la branche `main` par défaut. Si vous souhaitez changer la branche cible, vous pouvez modifier cette ligne dans le fichier `main.rs` :

```rust
let branch = "main";  
```

Remplacez `"main"` par le nom de la branche que vous souhaitez utiliser (par exemple `"master"` ou `"develop"`).

## Erreurs possibles

- **Git non installé** : Assurez-vous que Git est installé et accessible depuis le terminal.
- **Problèmes de permissions** : Si vous obtenez des erreurs liées aux permissions lors de l'exécution de `git-automate`, essayez d'exécuter la commande avec des privilèges d'administrateur (`sudo` sous Linux/MacOS).
- **Dépendances manquantes** : Assurez-vous que toutes les dépendances, notamment la crate `names`, sont correctement installées. Si nécessaire, réinstallez-les avec `cargo build`.

## Contribuer

Les contributions sont les bienvenues ! Si vous avez des idées d'amélioration, n'hésitez pas à forker le projet et à soumettre une pull request.

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.