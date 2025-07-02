

#include "log.h"

int main(int argc, char *argv[])
{
    setup_logging(LOG_LEVEL_DEBUG, "SAMPLE_APP");

    LOG_DEBUG("This is a debug message");
    LOG_INFO("This is an info message");
    LOG_WARN("This is a warning with %d", 10);
    LOG_ERROR("This is an error message %s", "hello world");

    return 0;
}