pipeline {
    agent any

    stages {
        stage('Prepation') {
            steps {
                // Setup cargo default stable version
                bat(/"C:\Users\Administrator\.cargo\bin\rustup" default stable/)
            }
        }
        stage('Lint') {
            steps {
                bat(/"C:\Users\Administrator\.cargo\bin\cargo" check/)
            }
        }
        stage('Test') {
            steps {
                bat(/"C:\Users\Administrator\.cargo\bin\cargo" test/)
            }
        }
        stage('Build') {
            steps {
                bat(/"C:\Users\Administrator\.cargo\bin\cargo" build/)
            }
            // post {
            //     success {
            //         // junit '**/target/surefire-reports/TEST-*.xml'
            //         // archiveArtifacts 'target/**/rust-test.exe'
            //         // stash name: "artifacts", includes: "artifacts/**/*"
            //         ([$class: 'CopyArtifact',
            //             projectName: '',
            //             filter: '',
            //             target: ''
            //         ])
            //     }
            // }
        }
        stage('Copy Archive') {
            steps {
                step ([$class: 'CopyArtifact',
                    projectName: 'p1',
                    filter: 'target/**/rust-*.exe'
                ]);
            }
        }
    }
}
