pipeline {
    agent any

    stages {
        stage('Build') {
            steps {
                bat(/"%CARGO_HOME\cargo%" --version/)
            }
        }
    }
}