pipeline {
    agent any

    tools {
        // Install the Maven version configured as "M3" and add it to the path.
        cargo "1.66.0"
    }

    stages {
        stage('Build') {
            steps {
                sh 'cargo --version'
            }
        }
    }
}