pipeline {
    agent any
    stages {
        stage("Build") {
            docker.image("node:4.1.2").inside {
                sh("echo test")
            }
        }
    }
}
