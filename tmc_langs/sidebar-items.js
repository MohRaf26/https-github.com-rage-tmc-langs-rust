initSidebarItems({"enum":[["ClientError","The main error type for tmc-client."],["ClientUpdateData","The update data type for the progress reporter."],["CommandError","An error caused by a failed attempt to execute an external command."],["ConfigValue","A setting in a TmcConfig file."],["DownloadResult",""],["LangsError","Main error type of the library."],["Language",""],["OutputFormat","Output formats for an archive."],["ParamError","Error validating TMC params values."]],"fn":[["check_exercise_updates","Checks the server for any updates for exercises within the given projects directory. Returns the ids of each exercise that can be updated."],["checkstyle","Checks the exercise's code quality."],["clean","Cleans the exercise."],["compress_project_to","Compresses the exercise to the target path."],["download_old_submission","Downloads the user's old submission from the server. Resets the exercise at the path before the download. If a submission_url is given, the current state of the exercise is submitted to that URL before the download."],["download_or_update_course_exercises","Downloads the given exercises, by either downloading the exercise template, updating the exercise or downloading an old submission. If the exercise doesn't exist on disk yet... if there are previous submissions and download_template is not set, the latest submission is downloaded. otherwise, the exercise template is downloaded. If the exercise exists on disk, it is updated using the course template."],["extract_project","Extracts the compressed project to the target location."],["extract_student_files","Extracts student files from the compressed exercise."],["find_exercise_directories","Finds valid exercises from the given path."],["get_available_points","Parses the available points from the exercise."],["get_course_data","Fetches the given course's details, exercises and course data."],["get_exercise_packaging_configuration","Gets the exercise packaging configuration."],["get_projects_dir","Returns the projects directory for the given client name. The return value for `my-client` might look something like `/home/username/.local/share/tmc/my-client` on Linux."],["get_setting","Fetches a setting from the config."],["get_settings","Fetches all the settings from the config."],["init_tmc_client_with_credentials","Initializes a TmcClient, using and returning the stored credentials, if any."],["list_local_course_exercises","Returns all of the exercises for the given course."],["login_with_password","Authenticates with the server, returning a login Token. Reads the password from stdin."],["login_with_token","Creates a login Token from a token string."],["migrate_exercise","Migrates an exercise from a location that's not managed by tmc-langs to the projects directory."],["move_projects_dir","Moves the projects directory from its current location to the target, taking all of the contained exercises with it."],["paste_exercise","Sends the paste to the server"],["prepare_solution","Note: used by tmc-server. Walks through each given path, processing files and copying them into the destination."],["prepare_stub","Prepares the exercise stub, copying tmc-junit-runner for Ant exercises."],["prepare_submission","Note: Used by tmc-server. Prepares a submission for further processing. The clone path is assumed to be a directory with the exercise name as the directory name, and the course name as its parent, ex. \"anything/some_course/some_exercise\""],["refresh_course","Used by tmc-server. Refreshes the course."],["reset","Resets the given exercise"],["reset_settings","Resets all settings in the config, removing those without a default value."],["run_tests","Runs tests for the exercise."],["scan_exercise","Scans the exercise."],["set_setting","Saves a setting in the config."],["sign_with_jwt","Signs the given serializable value with the given secret using JWT."],["submit_exercise","Submits the exercise to the server"],["unset_setting","Unsets the given setting."],["update_exercises","Updates the exercises in the local projects directory."]],"mod":[["file_util","Various utility functions, primarily wrapping the standard library's IO and filesystem functions"],["notification_reporter","Contains an utility for reporting warnings."]],"struct":[["CombinedCourseData",""],["Course","Information for a course."],["CourseConfig","A course configuration file. Contains information of all of the exercises of this course in the projects directory."],["CourseData","Data for a course."],["CourseDetails","Details for a course."],["CourseExercise","Exercise for a course."],["Credentials","Credentials for authenticating with tmc-server."],["DownloadOrUpdateCourseExercisesResult",""],["ExerciseDesc","A description of an exercise."],["ExerciseDetails","Details for an exercise."],["ExerciseDownload",""],["ExercisePackagingConfiguration","Represents configuration based on which submission may be packaged."],["FeedbackAnswer","Used to respond to feedback questions. See TmcClient::send_feedback."],["FileLockGuard","Guard that holds the locked file."],["LocalExercise","Exercise inside the projects directory."],["NewSubmission","Exercise submission."],["Organization","Organization information."],["ProjectsConfig",""],["ProjectsDirExercise","An exercise in the projects directory."],["RefreshData","Data from a finished course refresh."],["RefreshExercise","An exercise from a finished course refresh."],["Review","Code review."],["RunResult","The result of running an exercise's test suite against a submission."],["StyleValidationResult","The result of a style check."],["Submission","Exercise submission."],["SubmissionFeedbackResponse","Response to feedback."],["SubmissionFinished","Finished submission."],["TmcClient","A struct for interacting with the TestMyCode service, including authentication."],["TmcConfig","The main configuration file. A separate one is used for each client."],["TmcParams","TmcParams is used to safely construct data for a .tmcparams file, which contains lines in the form of export A=B export C=(D, E, F) the keys and values of the inner hashmap are validated to make sure they are valid as bash variables"],["UpdateResult","Updated exercises."]],"trait":[["LanguagePlugin","The trait that each language plug-in must implement."]],"type":[["Token","Authentication token."]]});