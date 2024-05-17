# Utiliser une image de base adaptée pour ARM
FROM rust:latest

# Installer les dépendances nécessaires
RUN apt-get update && \
    apt-get install -y \
    libgtk-3-dev \
    webkit2gtk-4.0 \
    libappindicator3-dev \
    librsvg2-dev \
    patchelf \
    libpcap-dev

# Définir le répertoire de travail
WORKDIR /app

# Copier le fichier de dépendances et installer les dépendances
COPY package.json yarn.lock ./
RUN yarn install

# Copier le reste du code
COPY . .

# Construire l'application
RUN yarn build && yarn tauri build

