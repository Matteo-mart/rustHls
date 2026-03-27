## Présentation du projet

Dans le cadre de mon apprentissage du langage Rust et des protocoles de streaming, j'ai développé rustHls.

Ce projet est un outil en ligne de commande (CLI) qui permet de transformer un fichier vidéo classique (ex: .mp4) en un flux HLS (HTTP Live Streaming). 
C'est la technologie utilisée par la plupart des plateformes de streaming pour adapter la qualité de la vidéo à la connexion de l'utilisateur.
## Fonctionnement Technique

Le programme fonctionne en trois étapes :

    Réception : prend une vidéo via la commande

    Traitement : utilise FFmpeg pour traiter la vidéo

    Indexation : génère un fichier .m3u8 qui sert de sommaire pour la lecture

### Pré-requis

    Rust & Cargo (pour compiler le projet).

    FFmpeg installé sur la machine (le projet s'appuie sur ses codecs).

## Installation et Test

    Clonage du dépôt :
    Bash

    git clone https://github.com/Matteo-mart/rustHls.git
    cd rustHls

    Compilation :
    Bash

    cargo build

    Exécution :
    Pour tester avec un fichier video.mp4 :
    Bash

    cargo run -- --input video.mp4 --output out/

## Structure du code

    main.rs : Point d'entrée de l'application et gestion des arguments.

    segmenter.rs : Logique de découpage et gestion des processus enfants.

    playlist.rs : Génération du texte au format M3U8.

## Perspectives d'évolution

    [ ] Ajouter une option pour choisir la résolution de sortie (720p, 1080p).

    [ ] Supprimer automatiquement les anciens segments pour économiser de l'espace disque.

    [ ] Créer une petite interface web en HTML/JS pour lire le flux généré.
