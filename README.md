# SDCManager

## Description

**SDCManager** est une application de gestion du matériel développée pour l’association **Son Des Cimes**. Ce logiciel de bureau facilite la gestion du prêt, de la location, de l’entretien et du renouvellement du matériel de l’association. 

L’application permet de suivre un inventaire synchronisé entre plusieurs utilisateurs, de gérer la disponibilité et la rentabilité des équipements, et de générer automatiquement devis, factures et notes de prêt.

---

## Fonctionnalités principales

- **Gestion du matériel :**  
  - Consultation sous forme de liste avec recherche, tri et filtres (catégorie, disponibilité, prix...)  
  - Consultation détaillée de chaque élément (nombre de sorties, rentabilité...)  
  - Emprunt et retour du matériel avec mise à jour automatique de la disponibilité  

- **Gestion des documents :**  
  - Création et modification de devis et feuilles d’emprunt  
  - Génération automatique des factures à partir des devis ou emprunts  

- **Fonctions administratives (mode Administrateur) :**  
  - Ajout, suppression et modification du matériel  
  - Personnalisation des formules de calcul des prix de location  
  - Modification des modèles de documents (devis, factures, emprunts)  
  - Gestion des utilisateurs et synchronisation  
  - Gestion des sauvegardes et restauration de versions antérieures de la base de données  

---

## Technologies utilisées

- **Interface utilisateur :** Vue.js (HTML, CSS, JavaScript)  
- **Backend & logique métier :** Rust via Tauri  
- **Base de données :** SQLite  
- **Synchronisation :** Peer-to-peer avec Syncthing  
- **Design :** Prototype réalisé sous Figma  

---

## Installation

Les exécutables d’installation sont disponibles dans le dossier [`release`](./release).

1. Télécharger l'exécutable correspondant à votre système d’exploitation (Windows, macOS, Linux).  
2. Lancer l’installation via l’exécutable, qui s’occupera d’installer et configurer automatiquement les dépendances nécessaires.  
3. Au premier lancement, échangez un code de synchronisation simple et intégré dans l’interface pour connecter votre application aux autres utilisateurs de l’association.  
4. Utilisez l’application même hors connexion : la synchronisation des données se fera automatiquement dès que la connexion sera disponible.

---

## Utilisation

- Navigation simple via menu de navigation sur le côté gauche. 
- Accès aux différentes sections :  
  - Accueil  
  - Matériel (liste, détail, emprunt/retour)  
  - Documents (devis, factures, emprunts)  
  - Administration (matériel, formules, modèles, utilisateurs, sauvegardes)  

---

## Contribution

Contributions, suggestions et rapports de bugs sont les bienvenus !  
Merci de bien vouloir ouvrir une issue avant de proposer une Pull Request.

---

## Licence

Ce projet est sous licence **MIT**. Voir le fichier [LICENSE](./LICENSE) pour plus de détails.
