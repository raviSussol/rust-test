pipeline {
    agent any

    stages {
        stage('Prepation') {
            steps {
                // Setup cargo default stable version
                bat(/"C:\Users\Administrator\.cargo\bin\rustup" default stable/)
            }
        }
        stage('Test') {
            steps {
                // Setup cargo default stable version
                bat(/"C:\Users\Administrator\.cargo\bin\cargo" test/)
            }
        }
    }
}
