pipeline {
    agent any
    environment {
        CG_HOME = 'C:\\Users\\Administrator\\.cargo\\bin'
    }
    stages {
        stage('Prepation') {
            steps {
                // Setup cargo default stable version
                bat(/"%CG_HOME%\rustup" default stable/)
            }
        }
        stage('Lint') {
            steps {
                bat(/"%CG_HOME%\cargo" check/)
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
            post {
                success {
            //         // junit '**/target/surefire-reports/TEST-*.xml'
                    archiveArtifacts 'target/**/rust-*.exe'
            //         // stash name: "artifacts", includes: "artifacts/**/*"
            //         ([$class: 'CopyArtifact',
            //             projectName: '',
            //             filter: '',
            //             target: ''
            //         ])
                }
            }
        }
        // stage('Copy Archive') {
        //     steps {
        //         step ([$class: 'CopyArtifact',
        //             projectName: 'p1',
        //             filter: 'target/debug/rust-test.exe'
        //         ]);
        //     }
        // }
    }
}
