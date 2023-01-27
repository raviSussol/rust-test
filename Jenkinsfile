pipeline {
    agent any

    stages {
        stage('Prepation') {
            steps {
                ws("C:\\Users\\Administrator\\.cargo\\bin") {
                    // Setup cargo default stable version
                    bat(/rustup default stable/)
                }
            }
        }
        stage('Lint') {
            steps {
                ws("C:\\Users\\Administrator\\.cargo\\bin") {
                    bat(/cargo check/)
                }
            }
        }
        stage('Test') {
            steps {
                ws("C:\\Users\\Administrator\\.cargo\\bin") {
                    bat(/cargo test/)
                }
            }
        }
        stage('Build') {
            steps {
                ws("C:\\Users\\Administrator\\.cargo\\bin") {
                    bat(/cargo build/)
                }
            }
        }
    }
}
