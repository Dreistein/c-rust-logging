#include "log.h"

#define BUFFER_SIZE 1024

void backend_log_callback(int level, const char *filename, int lineno, char *message);

__attribute__((format(printf, 4, 5))) void log_message(int level, const char *filename, int lineno, const char *fmt, ...)
{
    char buffer[BUFFER_SIZE];

    va_list args;
    va_start(args, fmt);
    vsnprintf(buffer, BUFFER_SIZE, fmt, args);
    va_end(args);

    backend_log_callback(level, filename, lineno, buffer);
}
