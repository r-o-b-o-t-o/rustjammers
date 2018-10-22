using System;
using UnityEngine;

namespace Ui
{
	public class MainMenuCamera : MonoBehaviour
	{
		[SerializeField] private Transform lookAtTarget;
		[SerializeField] private Transform pivot;
		[SerializeField] private Camera cam;
		[SerializeField] private float speed = 15.0f;
		[SerializeField] private float radius = 20.0f;

		private float angle = (float)(3.5 * Math.PI);

		private void Update ()
		{
			this.angle += Time.deltaTime * this.speed * (float)Math.PI / 180.0f;

			var x = this.pivot.position.x + this.radius * Mathf.Cos(this.angle);
			var z = this.pivot.position.z + this.radius * Mathf.Sin(this.angle);
			this.cam.transform.position = new Vector3(x, this.pivot.position.y, z);

			this.cam.transform.LookAt(this.lookAtTarget);
		}
	}
}
