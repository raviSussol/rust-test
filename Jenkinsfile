/* Requires the Docker Pipeline plugin */
pipeline {
    agent {
        table 'rust'
    }

    stages {
        stage('Hello') {
            steps {
                sh 'cargo --version'
            }
        }
    }
}