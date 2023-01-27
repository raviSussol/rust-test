pipeline {
    agent any

    stages {
        def cargoHome
        stage('Prepation') {
            cargoHome = "C:\\Users\\Administrator\\.cargo\\bin"
            steps {
                // ws("C:\\Users\\Administrator\\.cargo\\bin") {
                    // Setup cargo default stable version
                    // bat(/rustup default stable/)
                // }
                withEnv(["CG_HOME=$cargoHome"]) {
                    bat(/rustup default stable/)
                }
            }
        }
        stage('Build') {
            steps {
                ws("C:\\Users\\Administrator\\.cargo\\bin") {
                    bat(/cargo --version/)
                }
            }
        }
    }
}
