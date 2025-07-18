# SDCManager

## Description

**SDCManager** est une application de gestion du matériel développée pour l’association **Son Des Cimes**. Ce logiciel de bureau facilite la gestion du prêt, de la location, de l’entretien et du renouvellement du matériel de l’association. 

L’application permet de suivre un inventaire de materiel synchronisé entre plusieurs utilisateurs, de gérer la disponibilité et la rentabilité des équipements, et de créer et gérer des devis et factures, en utilisant l'inventaire de materiel qui gardera compte des sorties.


## Fonctionnalités principales

- **Gestion du matériel :**  
  - Consultation sous forme de liste avec recherche, tri et filtres (catégorie, disponibilité, prix...)  
  - Consultation détaillée de chaque élément (nombre de sorties, rentabilité...)  
  - Emprunt et retour du matériel avec mise à jour automatique de la disponibilité  

- **Gestion des documents :**  
  - Création et modification de devis
  - Génération automatique des factures à partir des devis
  - Visualisation des évenements à venir sous forme de planning

- **Synchronisation :**  
  - Synchronisation des données via un Google Drive centralisé
  - Envoi et récupération des données intégrées à l'application

- **Fonctions administratives (mode Administrateur) :**  
  - Ajout, suppression et modification du matériel  
  - Personnalisation des formules de calcul des prix de location

## Technologies utilisées

- **Interface utilisateur :** Vue.js (HTML, CSS, JavaScript)  
- **Backend :** Rust via Tauri  
- **Base de données :** SQLite  
- **Synchronisation :** Google Drive

## Installation

Les exécutables d’installation sont disponibles dans le dossier [`release`](./release).

1. Télécharger l'exécutable correspondant à votre système d’exploitation (Windows, macOS, Linux).  
2. Lancer l’installation via l’exécutable, qui s’occupera d’installer et configurer automatiquement les dépendances nécessaires.  
3. Pour synchroniser, allez dans les paramêtres dans la section "Synchronisation Drive" et suivez le guide (Compte Google requis).
4. Utilisez l’application même hors connexion : la synchronisation des données sera disponible dès que la connexion sera disponible.


## Utilisation

- Navigation simple via menu de navigation sur le côté gauche. 
- Accès aux différentes sections :   
  - Matériel (liste, détail, historique des sorties)  
  - Documents (devis, factures, planning)  
  - Administration (formules decontribution, données par défault)
  - Paramêtres (accessiblité, synchronisation)  

## Licence

Ce projet est sous licence **MIT**. Voir le fichier [LICENSE](./LICENSE) pour plus de détails.
