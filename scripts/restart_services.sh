#!/bin/bash
systemctl daemon-reload
systemctl restart coldvaults_auth
systemctl restart coldvaults_user
systemctl restart coldvaults_admin


