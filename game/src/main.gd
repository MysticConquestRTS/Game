extends Control

func _on_tutorial_pressed() -> void:
	pass

func _on_campaign_pressed() -> void:
	pass
	
func _on_multiplayer_pressed() -> void:
	pass

func _on_single_pressed() -> void:
	get_tree().change_scene_to_file("res://src/world/game.tscn")

func _on_options_pressed() -> void:
	pass

func _on_quit_pressed() -> void:
	get_tree().quit(0)
