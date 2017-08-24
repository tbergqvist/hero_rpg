var gulp = require('gulp');
var tsc = require('gulp-typescript');
var typescript = require('typescript');
var sourcemaps = require('gulp-sourcemaps');
var concat = require('gulp-concat');
var browserSync = require('browser-sync').create();

var tsProject = tsc.createProject("tsconfig.json");

gulp.task('moveJs', function () {
  return gulp.src(['src/**/*.js'])
  .pipe(gulp.dest('build/'));
});

gulp.task('moveHtml', function () {
  return gulp.src(['src/**/*.html'])
  .pipe(gulp.dest('build/'));
});

gulp.task('buildTS', ['moveJs'], function () {
  return tsProject.src()
  .pipe(sourcemaps.init())
  .pipe(tsProject())
  .pipe(sourcemaps.write(".", {sourceRoot: './'}))
  .pipe(gulp.dest('build'));
});

gulp.task('watch', ['buildTS', 'moveHtml'], function () {
  browserSync.init({
    server: "./",
    open: false
  });

  gulp.watch('src/**/*.ts', ['buildTS']);
  gulp.watch('src/**/*.tsx', ['buildTS']);
  gulp.watch('perf-main.ts', ['buildTS']);
  return gulp.watch('src/**/*.html', ['moveHtml']);
});