/* Requires the Docker Pipeline plugin */
pipeline {
    agent {
        label 'rust'
    }

    stages {
        stage('Hello') {
            steps {
                sh 'cargo --version'
            }
        }
    }
}