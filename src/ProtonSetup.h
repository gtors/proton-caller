//
// Created by avery on 28/10/2020.
//

#ifndef PROTON_CALLER_PROTONSETUP_H
#define PROTON_CALLER_PROTONSETUP_H
// For the future !
//#include <ncurses.h>
//#include <menu.h>

void setup(const char *reason);

void protonMenu();

void mkdir(const char *profile);

const char *findProfile();

void message();

void PRVersion();

#endif //PROTON_CALLER_PROTONSETUP_H
