#include <stdio.h>
#include <unistd.h>
#include <pthread.h>
#include "common.h"
#include "common_threads.h"

pthread_cond_t  c = PTHREAD_COND_INITIALIZER;
pthread_mutex_t m = PTHREAD_MUTEX_INITIALIZER;
int done = 0;

void *child(void *arg) {
    printf("child: begin\n");
    sleep(1);
    done = 1;
    printf("child: signal\n");
    Cond_signal(&c);
    return NULL;
}
int main(int argc, char *argv[]) {
    pthread_t p;
    printf("parent: begin\n");
    Pthread_create(&p, NULL, child, NULL);
    Mutex_lock(&m);
    printf("parent: check condition\n");
    while (done == 0) {
	sleep(2);
	printf("parent: wait to be signalled...\n");
	Cond_wait(&c, &m); 
    }
    Mutex_unlock(&m);
    printf("parent: end\n");
    return 0;
}

