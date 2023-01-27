/* Requires the Docker Pipeline plugin */
pipeline {
    agent {
        lable 'rust'
    }

    stages {
        stage('Hello') {
            steps {
                sh 'cargo --version'
            }
        }
    }
}