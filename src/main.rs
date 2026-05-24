// ============================================================
//  Gestionnaire de bibliothèque — correction commentée
// ============================================================
//  Pour compiler et exécuter :
//    rustc bibliotheque.rs -o bibliotheque && ./bibliotheque
// ============================================================

use std::fmt;

// ── 1. ENUM GENRE ────────────────────────────────────────────
//
// #[derive(Debug)] permet d'afficher avec {:?}
// PartialEq permet de comparer deux Genre avec ==
// Clone permet de copier la valeur (utile pour rechercher_par_genre)

#[derive(Debug, PartialEq, Clone)]
enum Genre {
    Roman,
    ScienceFiction,
    Biographie,
    Autre(String), // variante avec donnée : le nom du genre libre
}

// Affichage lisible pour Genre (impl Display)
impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Genre::Roman          => write!(f, "Roman"),
            Genre::ScienceFiction => write!(f, "Science-Fiction"),
            Genre::Biographie     => write!(f, "Biographie"),
            Genre::Autre(nom)     => write!(f, "{}", nom),
        }
    }
}

// ── 2. STRUCT LIVRE ──────────────────────────────────────────

struct Livre {
    titre:      String,
    auteur:     String,
    annee:      u32,
    genre:      Genre,
    disponible: bool,
}

impl Livre {
    // Constructeur : tous les livres commencent disponibles
    fn new(titre: &str, auteur: &str, annee: u32, genre: Genre) -> Self {
        Livre {
            titre:      String::from(titre),
            auteur:     String::from(auteur),
            annee,
            genre,
            disponible: true,
        }
    }
}

// Affichage lisible pour Livre
impl fmt::Display for Livre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dispo = if self.disponible { "✓ disponible" } else { "✗ emprunté" };
        write!(
            f,
            "«{}» — {} ({}) [{}] {}",
            self.titre, self.auteur, self.annee, self.genre, dispo
        )
    }
}

// ── 3. STRUCT BIBLIOTHÈQUE ───────────────────────────────────

struct Bibliotheque {
    livres: Vec<Livre>,
}

impl Bibliotheque {
    fn new() -> Self {
        Bibliotheque { livres: Vec::new() }
    }

    // Ajoute un livre à la collection
    fn ajouter(&mut self, livre: Livre) {
        println!("  + Ajout : {}", livre.titre);
        self.livres.push(livre);
    }

    // Emprunter : cherche le livre, vérifie sa dispo, le marque indisponible
    // Retourne Err(String) si introuvable ou déjà emprunté
    fn emprunter(&mut self, titre: &str) -> Result<(), String> {
        // iter_mut() pour pouvoir modifier le livre trouvé
        let livre = self.livres
            .iter_mut()
            .find(|l| l.titre == titre)
            .ok_or_else(|| format!("Livre «{}» introuvable", titre))?;
        //            ^^^^^^^^ transforme Option en Result, et ? propage l'erreur

        if !livre.disponible {
            return Err(format!("«{}» est déjà emprunté", titre));
        }

        livre.disponible = false;
        Ok(())
    }

    // Retourner : inverse d'emprunter
    fn retourner(&mut self, titre: &str) -> Result<(), String> {
        let livre = self.livres
            .iter_mut()
            .find(|l| l.titre == titre)
            .ok_or_else(|| format!("Livre «{}» introuvable", titre))?;

        if livre.disponible {
            return Err(format!("«{}» n'est pas marqué comme emprunté", titre));
        }

        livre.disponible = true;
        Ok(())
    }

    // Filtre et retourne les références vers les livres du genre demandé
    fn rechercher_par_genre(&self, genre: &Genre) -> Vec<&Livre> {
        self.livres
            .iter()
            .filter(|l| &l.genre == genre)
            .collect()
    }

    // Affiche tous les livres disponibles
    fn afficher_disponibles(&self) {
        let disponibles: Vec<&Livre> = self.livres
            .iter()
            .filter(|l| l.disponible)
            .collect();

        if disponibles.is_empty() {
            println!("  Aucun livre disponible.");
        } else {
            for livre in disponibles {
                println!("  {}", livre);
            }
        }
    }

    // Affiche tous les livres (bonus)
    fn afficher_tous(&self) {
        for livre in &self.livres {
            println!("  {}", livre);
        }
    }
}

// ── 4. FONCTION UTILITAIRE POUR AFFICHER LES RÉSULTATS ───────
//
// Gère le Result proprement au lieu d'utiliser unwrap() partout

fn afficher_resultat(action: &str, result: Result<(), String>) {
    match result {
        Ok(())   => println!("  ✓ {}", action),
        Err(msg) => println!("  ✗ Erreur — {}", msg),
    }
}

// ── 5. MAIN ──────────────────────────────────────────────────

fn main() {
    println!("=== Gestionnaire de bibliothèque ===\n");

    let mut biblio = Bibliotheque::new();

    // ── Ajout des livres ─────────────────────────────────────
    println!("[ Ajout des livres ]");
    biblio.ajouter(Livre::new("Dune",                  "Frank Herbert",    1965, Genre::ScienceFiction));
    biblio.ajouter(Livre::new("Le Nom de la Rose",     "Umberto Eco",      1980, Genre::Roman));
    biblio.ajouter(Livre::new("Sapiens",               "Yuval Harari",     2011, Genre::Biographie));
    biblio.ajouter(Livre::new("Fondation",             "Isaac Asimov",     1951, Genre::ScienceFiction));
    biblio.ajouter(Livre::new("Le Petit Prince",       "Antoine de Saint-Exupéry", 1943, Genre::Autre(String::from("Conte"))));

    // ── État initial ─────────────────────────────────────────
    println!("\n[ Catalogue complet ]");
    biblio.afficher_tous();

    // ── Emprunts ─────────────────────────────────────────────
    println!("\n[ Emprunts ]");
    afficher_resultat("Emprunt de Dune",            biblio.emprunter("Dune"));
    afficher_resultat("Emprunt de Sapiens",         biblio.emprunter("Sapiens"));
    afficher_resultat("Emprunt de Dune (2e fois)",  biblio.emprunter("Dune"));      // déjà emprunté
    afficher_resultat("Emprunt de '1984'",          biblio.emprunter("1984"));       // introuvable

    // ── Livres disponibles ───────────────────────────────────
    println!("\n[ Livres disponibles ]");
    biblio.afficher_disponibles();

    // ── Retour ───────────────────────────────────────────────
    println!("\n[ Retours ]");
    afficher_resultat("Retour de Dune",             biblio.retourner("Dune"));
    afficher_resultat("Retour de Fondation",        biblio.retourner("Fondation")); // pas emprunté

    // ── Recherche par genre ───────────────────────────────────
    println!("\n[ Science-Fiction disponible ]");
    let sf = biblio.rechercher_par_genre(&Genre::ScienceFiction);
    if sf.is_empty() {
        println!("  Aucun livre de science-fiction.");
    } else {
        for livre in sf {
            println!("  {}", livre);
        }
    }

    // ── État final ───────────────────────────────────────────
    println!("\n[ État final — disponibles ]");
    biblio.afficher_disponibles();
}