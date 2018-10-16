//Esteban

using System;
using System.Runtime.InteropServices;
using UnityEngine;
using UnityEngine.UI;


public class GameViewManagerScript : MonoBehaviour
{
	public Transform P1Transform;
	public Transform P1Hands;
	
	public Transform P2Transform;
	public Transform P2Hands;
	
	public Transform FrisbeeTransform;
	private bool frisbeeHeld = false;

	public Text P1Score;
	
	public Text P2Score;
	
	[StructLayout(LayoutKind.Sequential)]
	public struct ManagedState
	{
		public double p1_x;
		public double p1_y;
		public sbyte p1_score;

		public double p2_x;
		public double p2_y;
		public sbyte p2_score;

		public double zbee_x;
		public double zbee_y;
		public sbyte zbee_held;
	}

	private ManagedState MState;
	
	[DllImport("rustjammers_engine")]
	static extern void send_type_p1(IntPtr gameEngine,sbyte type1);
	
	[DllImport("rustjammers_engine")]
	static extern void send_type_p2(IntPtr gameEngine,sbyte type2);
	
	[DllImport("rustjammers_engine")]
	static extern IntPtr initialize();
	
	[DllImport("rustjammers_engine")]
	static extern void epoch(IntPtr gameEngine);
	
	[DllImport("rustjammers_engine")]
	static extern ManagedState get_state(IntPtr gameEngine);
	
	[DllImport("rustjammers_engine")]
	static extern void reset(IntPtr gameEngine);
	
	[DllImport("rustjammers_engine")]
	static extern void dispose(IntPtr gameEngine);
	
	//[DllImport("rustjammers_engine")]
	//static extern void sendInput(double x,double y,bool dash);

	private IntPtr currentGameEngine;
	
	
	void Start()
	{
		Debug.Log(DateTime.Now.ToString("yyyy/MM/dd HH:mm:ss.ff") + " - Initializing engine...");
		currentGameEngine = initialize();
		reset(currentGameEngine);
		MState =new ManagedState();
		send_type_p1(currentGameEngine, PlayerType.MyPlayersType.typeP1);
		send_type_p2(currentGameEngine, PlayerType.MyPlayersType.typeP2);
		Debug.Log(DateTime.Now.ToString("yyyy/MM/dd HH:mm:ss.ff") + " - Engine ready [" + currentGameEngine + "].");
	}
	
	void Update ()
	{
		epoch(currentGameEngine);
		MState = get_state(currentGameEngine);
		P1Score.text = ""+MState.p1_score;
		P2Score.text = ""+MState.p2_score;
		P1Transform.position = new Vector3((float)MState.p1_x, P1Transform.position.y,(float)MState.p1_y);
		P2Transform.position = new Vector3((float)MState.p2_x, P2Transform.position.y,(float)MState.p2_y);
		if (!frisbeeHeld) {
			if (MState.zbee_held == 0) {
				FrisbeeTransform.parent = P1Hands;
				FrisbeeTransform.localPosition = Vector3.zero;
				frisbeeHeld = true;
			} else if (this.MState.zbee_held == 1) {
				FrisbeeTransform.parent = P2Hands;
				FrisbeeTransform.localPosition = Vector3.zero;
				frisbeeHeld = true;
			}
		} else {
			if (MState.zbee_held == -1) {
				FrisbeeTransform.parent = null;
				frisbeeHeld = false;
			}
		}
		if (!frisbeeHeld) {
			FrisbeeTransform.position = new Vector3((float)MState.zbee_x, P2Transform.position.y,(float)MState.zbee_y);
		}
		/*
			if (PlayerType.MyPlayersType.typeP1 == 0 || PlayerType.MyPlayersType.typeP2 == 0)
			{
				bool dashed = false;
				double moveHorizontal = Input.GetAxis ("Horizontal");
				double moveVertical = Input.GetAxis ("Vertical");
				if (Input.GetKeyDown("space"))
				{
					dashed = true;
				}
				sendInput(moveHorizontal,moveVertical,dashed);
			}
		*/
	}


	private void OnDestroy()
	{
		Debug.Log(DateTime.Now.ToString("yyyy/MM/dd HH:mm:ss.ff") + " - Destroying engine...");
		dispose(currentGameEngine);
	}
}

