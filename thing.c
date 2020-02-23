#include <unistd.h>
#include <stdio.h>
#include <sys/wait.h>

int main() {
    pid_t child_pid;
    if ((child_pid = fork())) {
        printf("Parent. Child PID is %d\n", child_pid);
        waitpid(child_pid, NULL, 0);
    } else {
        printf("Child process\n");
        sleep(1);
    }
    return 0;
}