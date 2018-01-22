pipeline {
    agent any

    environment {
        REPO_SERVER = 'repo.youkebox.be'
        REPO_PATH   = '/var/vhosts/repo/${BUILD}'
    }

    stages {
        stage('Build') {
            steps {
                sh 'make'
                archiveArtifacts artifacts: 'target/release', fingerprint: true
            }
        }

        stage('Package') {
            steps {
                sh 'make package'
            }
        }

        stage('Deploy') {
            steps {
                sh 'scp youkebox-*.rpm root@${REPO_SERVER}:${REPO_PATH}/${BUILD}/packages/'
                sh 'ssh root@${REPO_SERVER} "createrepo --update ${REPO_PATH}/${BUILD}"'
            }
        }

        stage('Cleanup') {
            steps {
                sh 'make clean'
            }
        }
    }
}