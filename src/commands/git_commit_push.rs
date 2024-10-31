// use git2::{Error, Repository};
// use std::path::Path;

// pub fn git_commit_push(repo_path: &Path) -> Result<(), Error> {
//     // Abra o repositório
//     let repo = Repository::open(repo_path)?;

//     // Obtenha o índice do repositório
//     let mut index = repo.index()?;

//     // Verifique se há modificações
//     let status = repo.statuses(None)?;
//     let mut has_changes = false;
//     for entry in status.iter() {
//         if entry.status() != git2::Status::INDEX_NEW {
//             has_changes = true;
//             break;
//         }
//     }

//     // Se não há mudanças, retorne sem fazer nada
//     if !has_changes {
//         println!("No changes to commit.");
//         return Ok(());
//     }

//     // Adicione as mudanças ao índice
//     index.add_all(["*"].iter(), git2::AddOption::DEFAULT, None)?;

//     // Crie uma árvore de mudanças
//     let tree = index.write_tree()?;
//     let tree = repo.find_tree(tree)?;

//     // Obtenha o commit anterior (HEAD)
//     let head = repo.head()?;
//     let parent_commit = repo.find_commit(
//         head.target()
//             .ok_or_else(|| Error::from_str("HEAD not found"))?,
//     )?;

//     // Crie o commit
//     let signature = repo.signature()?;
//     let commit = repo.commit(
//         Some("HEAD"),       // Referência do commit
//         &signature,         // Autor
//         &signature,         // Committer
//         "Automated commit", // Mensagem do commit
//         &tree,              // Árvore atual
//         &[&parent_commit],  // Pais do commit
//     )?;

//     println!("Committed changes: {}", commit.id());

//     // Obtenha o remote 'origin'
//     let mut remote = repo.find_remote("origin")?;
//     remote.push(&["refs/heads/master:refs/heads/master"], None)?;

//     println!("Pushed changes to remote.");

//     Ok(())
// }
