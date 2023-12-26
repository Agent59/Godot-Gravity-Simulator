class_name Mass

extends RigidBody2D

# Relevant to be processed by the Space node for the gravity calculation.
var resource_name = "Mass"

# Called when the node enters the scene tree for the first time.
func _ready():
	pass

# Merges two colliding Masses into one.
func _on_body_entered(body):
	if body.get("resource_name") != "Mass":
		return
	
	# The body with the lower mass is removed.
	# If they have the same mass, the RID is used.
	# RID is used to remove only one of the masses
	# because they can never have the same RID.
	if (self.mass < body.mass \
	or self.mass == body.mass && RID(self) > RID(body)):
		self.queue_free()
		return
	
	# TODO The commented out, moves the merged node to the center of mass.
	# TODO But this leads two buggy collisions.
	# TODO Also the velocities should be merged.
	var total_m = self.mass + body.mass
	# var new_x = (self.mass * self.position.x + body.mass * body.position.x) / total_m
	# var new_y = (self.mass * self.position.y + body.mass + body.position.y) / total_m
	# self.position = Vector2(new_x, new_y)
	self.set_mass(total_m)
