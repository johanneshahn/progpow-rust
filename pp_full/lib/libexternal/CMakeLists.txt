set(SOURCES progpow.h progpow.cpp)

include_directories(BEFORE ..)

add_library(ppow SHARED ${SOURCES})
target_link_libraries(ppow ethcore ethash progpow)

if(ETHASHCL)
	target_link_libraries(ppow ethash-cl)
endif()
if(ETHASHCUDA)
	target_link_libraries(ppow ethash-cuda)
endif()
