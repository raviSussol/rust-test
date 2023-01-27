pipeline {
    agent {
        node {
            customWorkspace "C:\Users\Administrator\.cargo\bin"
        }
    }

    stages {
        stage('Prepation') {
            steps {
                // Setup cargo default stable version
                bat(/rustup default stable/)
            }
        }
        stage('Build') {
            steps {
                bat(/"C:\Users\Administrator\.cargo\bin\cargo" --version/)
            }
        }
    }
}
