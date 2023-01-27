pipeline {
    agent any

    stages {
        def cargoHome
        stage('Prepation') {
            cargoHome = "C:\Users\Administrator\.cargo\bin"
            steps {
                withEnv(["CG_HOME=$cargoHome"]) {
                    bat(/"%CG_HOME%\rustup" default stable/)
                }
            }
            // Setup cargo default stable version
            // bat(/"C:\Users\Administrator\.cargo\bin\rustup" default stable/)
        }
        stage('Build') {
            steps {
                bat(/"C:\Users\Administrator\.cargo\bin\cargo" --version/)
            }
        }
    }
}