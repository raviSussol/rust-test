/* Requires the Docker Pipeline plugin */
pipeline {
    agent any
    stages {
        stage('Lint') {
            steps {
                sh 'cargo fmt -- --check'
            }
        }
        stage('Test') {
            steps {
                sh 'cargo test'
            }
        }
        stage('Build') {
            steps {
                sh 'cargo build'
            }
        }
    }
}
