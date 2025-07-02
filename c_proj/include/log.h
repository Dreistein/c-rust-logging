#ifndef LOG_H
#define LOG_H

#include <stdio.h>
#include <stdarg.h>
#include <time.h>
#include <string.h>

#define LOG_LEVEL_DEBUG 0
#define LOG_LEVEL_INFO 1
#define LOG_LEVEL_WARN 2
#define LOG_LEVEL_ERROR 3

#define LOG_LEVEL LOG_LEVEL_DEBUG

void setup_logging(int level, const char *project_name);

void log_message(int level, const char *filename, int lineno, const char *fmt, ...) __attribute__((format(printf, 4, 5)));

#define LOG_DEBUG(fmt, ...)                                                   \
    do                                                                        \
    {                                                                         \
        log_message(LOG_LEVEL_DEBUG, __FILE__, __LINE__, fmt, ##__VA_ARGS__); \
    } while (0)

#define LOG_INFO(fmt, ...)                                                   \
    do                                                                       \
    {                                                                        \
        log_message(LOG_LEVEL_INFO, __FILE__, __LINE__, fmt, ##__VA_ARGS__); \
    } while (0)

#define LOG_WARN(fmt, ...)                                                   \
    do                                                                       \
    {                                                                        \
        log_message(LOG_LEVEL_WARN, __FILE__, __LINE__, fmt, ##__VA_ARGS__); \
    } while (0)

#define LOG_ERROR(fmt, ...)                                                   \
    do                                                                        \
    {                                                                         \
        log_message(LOG_LEVEL_ERROR, __FILE__, __LINE__, fmt, ##__VA_ARGS__); \
    } while (0)

#endif
