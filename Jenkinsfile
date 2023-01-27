pipeline {
    agent any

    stages {
        stage('Prepation') {
            steps {
                // withEnv(["CG_HOME=C:\\Users\\Administrator\\.cargo\\bin"]) {
                //     bat(/"%CG_HOME\rustup%" default stable/)
                // }
                // ws("C:\\Users\\Administrator\\.cargo\\bin") {
                    // Setup cargo default stable version
                    bat(/"C:\Users\Administrator\.cargo\bin\rustup" default stable/)
                // }
            }
        }
        stage('Lint') {
            // steps {
            //     // ws("C:\\Users\\Administrator\\.cargo\\bin") {
            //         bat(/cargo check/)
            //     // }
            // }
                bat(/"C:\Users\Administrator\.cargo\bin\cargo" check/)
            // withEnv(["CG_HOME=C:\\Users\\Administrator\\.cargo\\bin"]) {
            // }
        }
        // stage('Test') {
        //     steps {
        //         ws("C:\\Users\\Administrator\\.cargo\\bin") {
        //             bat(/cargo test/)
        //         }
        //     }
        // }
        // stage('Build') {
        //     steps {
        //         ws("C:\\Users\\Administrator\\.cargo\\bin") {
        //             bat(/cargo build/)
        //         }
        //     }
        // }
    }
}
