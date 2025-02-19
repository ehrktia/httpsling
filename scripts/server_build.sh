#! /usr/bin/env bash
echo "building server...."
basedir=`pwd`
echo "using ${basedir} as base.."
go build -ldflags "-s -w" -o "${basedir}"/bin/server "${basedir}"/go-server/main.go
ls -lart "${basedir}"/bin
du -h "${basedir}"/bin/server
