pipeline {
    agent any
    environment {
        CG_HOME = 'C:\\Users\\Administrator\\.cargo\\bin'
    }
    stages {
        stage('Checkout SCM') {
            steps {
                // checkout([
                //     $class: 'GitSCM',
                //     branches: [[name: 'main']],
                //     userRemoteConfigs: [[
                //         url: 'https://github.com/raviSussol/rust-test.git',
                //         credentialsId: 'rust-test-cred-id',
                //     ]]
                // ])
                checkout([
                    $class: 'GitSCM',
                    branches: [[name: '*/main']],
                    doGenerateSubmoduleConfigurations: false,
                    extensions: [[$class: 'CleanCheckout']],
                    submoduleCfg: [],
                    userRemoteConfigs: [[
                        credentialsId: '8f9637c7-a256-4013-b10a-02ab5962891',
                        url: 'https://github.com/raviSussol/rust-test.git'
                    ]]
                ])
            }
        }
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
                bat(/"%CG_HOME%\cargo" test/)
            }
        }
        stage('Build') {
            steps {
                bat(/"%CG_HOME%\cargo" build/)
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
