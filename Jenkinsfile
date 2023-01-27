/* Requires the Docker Pipeline plugin */
pipeline {
    agent any

    stages {
        stage('Hello') {
            steps {
                cargo build
            }
        }
    }
}