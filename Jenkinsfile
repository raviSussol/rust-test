/* Requires the Docker Pipeline plugin */
pipeline {
    agent any

    stages {
        stage('Hello') {
            steps {
                bat("cargo --version")
            }
        }
    }
}