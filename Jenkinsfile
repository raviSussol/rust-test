pipeline {
    agent any

    stages {
        stage('Build') {
            steps {
                bat(/"C:\Users\Administrator\.cargo\bin\rustup" default stable/)
                bat(/"C:\Users\Administrator\.cargo\bin\cargo" --version/)
            }
        }
    }
}