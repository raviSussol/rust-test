/* Requires the Docker Pipeline plugin */
pipeline {
    agent {
        docker { image 'rust:1.66.0-alpine' }
    }

    stages {
        stage('Hello') {
            steps {
                sh 'cargo --version'
            }
        }
    }
}